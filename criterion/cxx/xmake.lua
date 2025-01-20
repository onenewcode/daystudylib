add_rules("mode.release", "mode.debug")

target("benchmark_test")
    set_kind("binary")
    add_files("main.cpp")
    
    -- 添加对Google Benchmark库的链接
    add_links("benchmark")
    
    -- 指定Google Benchmark库的搜索路径
    -- 如果你通过APT安装了Google Benchmark，它通常位于/usr/lib
    add_linkdirs("/usr/local/lib") -- 根据实际情况调整路径
    
    -- 如果需要，也可以添加其他必要的库，例如pthread
    add_syslinks("pthread")
    
    -- 启用必要的编译选项以支持AVX2和FMA指令集
    add_cxxflags("-mavx2", "-mfma")
target("test")
    set_kind("binary")
    add_files("main.cpp")
    
    -- 添加对Google Benchmark库的链接
    add_links("benchmark")
    
    -- 指定Google Benchmark库的搜索路径
    -- 如果你通过APT安装了Google Benchmark，它通常位于/usr/lib
    add_linkdirs("/usr/local/lib") -- 根据实际情况调整路径
    
    -- 如果需要，也可以添加其他必要的库，例如pthread
    add_syslinks("pthread")
    
    -- 启用必要的编译选项以支持AVX2和FMA指令集
    add_cxxflags("-mavx2", "-mfma")
target("asm")
    add_files("main.cpp")
    
    -- 启用必要的编译选项以支持AVX2和FMA指令集
    add_cxxflags("-mavx2", "-mfma")
    
    -- 使用 -march=native 选项自动检测并启用当前CPU支持的所有指令集
    add_cxxflags("-march=native")
    
    -- 添加生成汇编文件的编译选项
    add_rules("mode.debug", "mode.release")
    
    -- 使用 after_build 钩子来生成汇编文件
    after_build(function (target)
        -- 获取源文件路径
        local source_file = target:sourcefiles()[1]
        -- 指定汇编文件的输出路径
        local asm_file = "/home/ztf/daystudylib/criterion/cxx/main.s"
        -- 调用 GCC 生成汇编文件
        os.run('g++ -S -mavx2 -mfma -march=native -masm=intel -o "%s" "%s"', asm_file, source_file)
    end)