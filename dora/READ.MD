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