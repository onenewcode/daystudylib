# 运行
py
>pip install dora-rs     
>dora run dataflow.yml

rust要提前编译不是很方便
提前运行，但是可能会导致写同时写单个文件导致冲突，可能需要添加run.rs脚本
> dora build .\dataflow.yml

>
# conda 
**查看环境**：
>conda env list

**创建环境**：
>conda create -n env_name python=3.12

**激活环境**：
>conda activate env_name

# 跨机通信
## 主机
首先获取ip地址，以下使用xxxxx代替本机ip地址
首先在PC内启动dora coordinator:

>dora coordinator
再新建一个命令行窗口，启动dora daemon:(这里的IP地址替换为前面记下的IP地址)

>dora daemon -c 192.168.3.183

## Pi
然后在Pi内启动dora daemon:
>dora daemon -c 192.168.3.183 --machine-id pi

## yaml编写
在Dora-Camp-Tutorial/lab1/Multiple-PC文件夹中已写好一个ctrl_arm.yml文件，内容如下：

# Run on PC
```yaml
nodes:
  - id: key-listener
    path: ../../src/key_listener.py
    inputs:
      tick: dora/timer/millis/10
    outputs:
      - char

  - id: key-text
    path: ../../src/key_text.py
    inputs:
      keyboard: key-listener/char
    outputs:
      - text

  - id: trans-cmd
    path: ../../src/trans_cmd.py
    inputs:
      key-keyboard: key-text/text
    outputs:
      - movec
      - claw
      - save
      - clear
      - begin
      - stop
      - goto

  - id: arm
    path: ./src/gen72.py  # The relative path of gen72.py that relative to the path where you started the dora daemon on pi
    _unstable_deploy:
      machine: pi
    inputs:
      movec: trans-cmd/movec
      claw: trans-cmd/claw
      save: trans-cmd/save
      clear: trans-cmd/clear
      begin: trans-cmd/begin
      stop: trans-cmd/stop
      goto: trans-cmd/goto
    env:
      ROBOT_IP: 192.168.1.18  # gen72 robotic arm default IP address
      SAVED_POSE_PATH: ./recorder/pose_library.json # The relative path of pose_library.json that relative to the path where you started the dora daemon on pi
```
相比于实验1-1，实验1-2的大部分节点都是运行在PC上的，如 key-listener、key-text、trans-cmd，只有arm节点是运行在Pi上的。在实验1-1中，dora的coordinater和daemon都运行在Pi中，而实验1-2中是PC运行：coordinater+daemon，Pi只运行一个daemon。

这里详细解析arm节点。由于该节点内加上了下面这个键和键值：
```yaml
_unstable_deploy:
    machine: pi
```
故表示该节点将运行在，名为pi的机器上。在dora start该yaml文件后，coordinator便会去尝试找到该机器上启动的daemon。pi机器上的daemon再根据path来找到pi上的代码或包，然后启动该节点。

该节点的path路径和代码内打开文件的路径都将根据在Pi上启动daemon的路径。
