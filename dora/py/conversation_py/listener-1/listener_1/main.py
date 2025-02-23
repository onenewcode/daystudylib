from dora import Node


def main():
    node = Node()
    for event in node:
        # 判断是否是 INPUT 事件
        if event["type"] == "INPUT":
            # 取出事件中的 vslue 值，转为 py 类型 ,如何通过类型或者字符串？
            message = event["value"][0].as_py()
            print(f"""I heard {message} from {event["id"]}""")


if __name__ == "__main__":
    main()
