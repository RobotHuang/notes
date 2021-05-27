# Ceph手工部署



全手工安装ceph环境，打造最精简的单机ceph实验环境

### 操作系统相关

操作系统镜像地址



https://mirrors.aliyun.com/centos/7.7.1908/isos/x86_64/CentOS-7-x86_64-Minimal-1908.iso



```

TYPE="Ethernet"

PROXY_METHOD="none"

BROWSER_ONLY="no"

BOOTPROTO="static"

NAME="enp0s3"

UUID="e18f3b76-968b-4f26-9ca8-b38dc4823967"

DEVICE="enp0s3"

ONBOOT="yes"

IPADDR="192.168.0.201"

GATEWAY="192.168.0.1"

NETMASK="255.255.255.0"

DNS1="223.5.5.5"

```



Yum配置文件如下



```

wget -O /etc/yum.repos.d/CentOS-Base.repo http://mirrors.aliyun.com/repo/Centos-7.repo

sed -i -e '/mirrors.cloud.aliyuncs.com/d' -e '/mirrors.aliyuncs.com/d' /etc/yum.repos.d/CentOS-Base.repo

wget -O /etc/yum.repos.d/epel.repo http://mirrors.aliyun.com/repo/epel-7.repo



```



 o 内容如下



```

[ceph-noarch]

name=Ceph noarch packages

baseurl=https://mirrors.aliyun.com/ceph/rpm-nautilus/el7/noarch/

enabled=1

gpgcheck=1

type=rpm-md

gpgkey=https://mirrors.aliyun.com/ceph/keys/release.asc





[ceph]

name=Ceph packages for $basearch

baseurl=https://mirrors.aliyun.com/ceph/rpm-nautilus/el7/$basearch

enabled=1

priority=2

gpgcheck=1

type=rpm-md

gpgkey=https://mirrors.aliyun.com/ceph/keys/release.asc

```



安装好ntp服务，确保时钟一致



```

yum install ntp ntpdate

```



准备好主机名解析



```

[root@centos1 ~]# cat /etc/hosts

......

192.168.0.201 centos1 centos1.local
172.17.0.13 ceph1 ceph1.local
172.17.0.14 ceph2 ceph2.local
172.17.0.15 ceph3 ceph3.local

```



### ceph安装



#### 安装ceph软件包



```

yum install ceph ceph-radosgw

```



参考文档https://docs.ceph.com/docs/master/install/manual-deployment/

准备ceph.conf配置文件，注意路径为/etc/ceph/ceph.conf



```

[global]

fsid = 797be698-f819-4eb8-a459-6cb92502b7f9

mon initial members = ceph1

mon host = 192.168.0.201

public network = 192.168.0.0/24

cluster network = 192.168.0.0/24

auth cluster required = cephx

auth service required = cephx

auth client required = cephx

osd pool default size = 1

osd pool default min size = 1

osd crush chooseleaf type = 0 

```
```
[global]
fsid = 797be698-f819-4eb8-a459-6cb92502b7f9

mon initial members = ceph1

mon host = 172.17.0.13

public network = 172.17.0.0/20

cluster network = 172.17.0.0/20

auth cluster required = cephx

auth service required = cephx

auth client required = cephx

osd pool default size = 1

osd pool default min size = 1

osd crush chooseleaf type = 0
```


#### MON安装



```

ceph-authtool --create-keyring /tmp/ceph.mon.keyring --gen-key -n mon. --cap mon 'allow *'

sudo ceph-authtool --create-keyring /etc/ceph/ceph.client.admin.keyring --gen-key -n client.admin --cap mon 'allow *' --cap osd 'allow *' --cap mds 'allow *' --cap mgr 'allow *'

sudo ceph-authtool --create-keyring /var/lib/ceph/bootstrap-osd/ceph.keyring --gen-key -n client.bootstrap-osd --cap mon 'profile bootstrap-osd' --cap mgr 'allow r'

sudo ceph-authtool /tmp/ceph.mon.keyring --import-keyring /etc/ceph/ceph.client.admin.keyring

sudo ceph-authtool /tmp/ceph.mon.keyring --import-keyring /var/lib/ceph/bootstrap-osd/ceph.keyring

sudo chown ceph:ceph /tmp/ceph.mon.keyring

monmaptool --create --add node1 192.168.0.201 --fsid 797be698-f819-4eb8-a459-6cb92502b7f9 /tmp/monmap

sudo -u ceph mkdir /var/lib/ceph/mon/ceph-ceph1

sudo -u ceph ceph-mon --mkfs -i ceph1 --monmap /tmp/monmap --keyring /tmp/ceph.mon.keyring

systemctl start ceph-mon@ceph1

systemctl enable ceph-mon@ceph1



```



#### 安装OSD



参考

https://github.com/ceph/ceph/tree/master/src/ceph-volume/ceph_volume

https://docs.ceph.com/docs/master/install/manual-deployment/#bluestore



```

ceph-volume lvm list

sudo ceph-volume lvm create --data /dev/sdb

```



#### 安装MGR



```

mkdir /var/lib/ceph/mgr/ceph-ceph1

ceph auth get client.bootstrap-mgr -o /var/lib/ceph/bootstrap-mgr/ceph.keyring

ceph --cluster ceph --name client.bootstrap-mgr --keyring /var/lib/ceph/bootstrap-mgr/ceph.keyring auth get-or-create mgr.ceph1 mon "allow profile mgr" osd "allow *" mds "allow *" -o /var/lib/ceph/mgr/ceph-ceph1/keyring

touch /var/lib/ceph/mgr/ceph-ceph1/done

touch /var/lib/ceph/mgr/ceph-ceph1/systemd

chown ceph:ceph -R /var/lib/ceph/mgr/ceph-ceph1

systemctl start  ceph-mgr@ceph1

systemctl enable  ceph-mgr@ceph1



```



修复1 monitors have not enabled msgr2



```

ceph mon enable-msgr2

```




作者：秦商牧羊
https://www.bilibili.com/read/cv5926591
出处： bilibili