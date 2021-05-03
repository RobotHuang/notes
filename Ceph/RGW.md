# 对象存储网关RGW
> https://docs.ceph.com/en/latest/radosgw/layout/
> https://blog.csdn.net/lzw06061139/article/details/107246059
> https://ypdai.github.io/2018/11/30/rgw%E9%87%8C%E9%9D%A2%E7%94%A8%E6%88%B7%E3%80%81bucket%E3%80%81%E7%94%A8%E6%88%B7%E6%95%B0%E6%8D%AE%E4%B9%8B%E9%97%B4%E5%85%B3%E7%B3%BB/
> https://blog.csdn.net/lzw06061139/article/details/107410287
> https://blog.csdn.net/lzw06061139/article/details/107619544#comments_13788368
## 数据组织和存储
基本概念：
* 用户
* 存储桶
* 对象

RGW为了兼容 Amazon S3 和 OpenStack Swift 接口，RGW 将用户分为用户和子用户，用户对应 s3 的用户，子用户是 swift 用户。Jewel 版本之后引入 tenant 概念，不同 tenant 之间的用户和存储桶互相隔离。

数据存储在RADOS对象中有三种形式
1. 以二进制文件方式保存
2. 以键值对方式保存在 xattr 中
3. 以键值对的方式保存在 omap 中

### 用户
用户的信息包含用户认证信息，访问控制权限信息和配额信息

#### S3认证
1. 使用用户私有密钥（secret_key），请求内容等，采用与RGW网关约定好的算法计算出数字签名后，将数字签名以及用户访问密钥（access_key）封装在请求中发送给 RGW 网关。
2. RGW 网关收到请求后，使用用户访问密钥作为索引从 RADOS 集群中读取用户信息，并从用户信息中获取到用户的私有密钥
3. 使用用户私有密钥，请求内容等，采用与应用约定好的算法计算数字签名
4. 判断 RGW 生成的数字签名与请求的签名是否匹配，如果匹配，则认为请求是真实的，用户认证通过。

RGWUserInfo 结构
```cpp
struct RGWUserInfo
{
  rgw_user user_id; // 用户id，包含tenant和id
  string display_name; // 用户名
  string user_email;
  map<string, RGWAccessKey> access_keys; // 用户访问密钥，id和key
  map<string, RGWAccessKey> swift_keys;
  map<string, RGWSubUser> subusers;
  __u8 suspended;
  int32_t max_buckets;
  uint32_t op_mask; // 用户操作访问权限
  RGWUserCaps caps; // 授权用户权限，由一组<caps-type, perm>组成，caps-type是users, buckets, etc. perm是read, write, etc.
  __u8 admin;
  __u8 system;
  rgw_placement_rule default_placement;
  list<string> placement_tags;
  RGWQuotaInfo bucket_quota; // 存储桶的一些限制
  map<int, string> temp_url_keys;
  RGWQuotaInfo user_quota; // 用户下的一些限制
  uint32_t type;
  set<string> mfa_ids;
  string assumed_role_arn;
  
  //...
}
```
RGW 在 RADOS 中使用二级索引来存储用户信息，第一级索引是：访问密钥，子用户，email。对象数据部分存储的信息是用户ID。第二级索引是：用户ID，对象数据部分存储的信息是用户信息

### 存储桶
一个存储桶对应一个 RADOS 对象。存储桶有两类信息：一类是对 RGW 网关透明的信息，通常是用户自定义的元数据；一类是 RGW 网关关注的信息，包括存储桶中对象的存储策略、存储桶中索引对象的数目以及应用对象与索引对象的映射关系、存储桶的配额等。
```cpp
struct RGWBucketInfo {
  rgw_bucket bucket;
  rgw_user owner;
  uint32_t flags{0};
  string zonegroup;
  ceph::real_time creation_time;
  rgw_placement_rule placement_rule;
  bool has_instance_obj{false};
  RGWObjVersionTracker objv_tracker; /* we don't need to serialize this, for runtime tracking */
  RGWQuotaInfo quota;

  // layout of bucket index objects
  rgw::BucketLayout layout;

  // Represents the number of bucket index object shards:
  //   - value of 0 indicates there is no sharding (this is by default
  //     before this feature is implemented).
  //   - value of UINT32_T::MAX indicates this is a blind bucket.

  // Represents the shard number for blind bucket.
  const static uint32_t NUM_SHARDS_BLIND_BUCKET;
```
在创建存储桶时，RGW 网关会同步创建一个或多个索引对象，用于保存改存储桶下的对象列表，因此在存储桶中有新的对象上传的时候必须更新索引对象。在Hammer 版本之前，对于单个存储桶只创建一个索引对象。而之后版本将索引对象进行分片（shard），把一个索引对象切分成多个（通过 rgw_override_bucket_index_max_shard 指定）对象，但是分片会影响读的性能，RGW 网关改为并发读（通过 rgw_bucket_index_max_aio 配置项可调整并发数）以降低查询存储桶对象列表操作的处理时间。
### 对象
RGW 对单个对象提供了两种上传方式：整体上传和分段上传。整体上传的对象大小不能超过5GB（可通过配置项 rgw_max_put_size 调整）。
* rgw_max_chunk_size: 存到 RADOS 集群中最大块的大小
* rgw_obj_stripe_size: 用来指定当一个对象被分成多个 RADOS 对象时中间对象的大小
* Class RGWObjManifest: 用来管理用户上传的对象和 RADOS 对象的对应关系