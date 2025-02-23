from dora import Node
import pyarrow as pa


def main():
    node = Node()

    for event in node:
        # 设置事件类型
        if event["type"] == "INPUT":
            print(
                f"""Node received:
            id: {event["id"]},
            value: {event["value"]},
            metadata: {event["metadata"]}"""
            )
            # 设置传递数据
            node.send_output("speech", pa.array(["Hello World"]))


if __name__ == "__main__":
    main()
