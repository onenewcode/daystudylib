from dora import Node
import cv2

def main():
    node = Node()

    for event in node:
        if event["type"] == "INPUT":
            event_id = event["id"]
            if event_id == "image":
                data = event["value"]
                metadata = event["metadata"]

                # 转换为 NumPy 数组
                np_array = data.to_numpy()

                # 根据编码处理数据
                encoding = metadata["encoding"]
                if encoding in ["bgr8", "rgb8"]:
                    # 原始像素格式
                    height = metadata["height"]
                    width = metadata["width"]
                    image = np_array.reshape((height, width, 3))
                    if encoding == "rgb8":
                        image = cv2.cvtColor(image, cv2.COLOR_RGB2BGR)
                elif encoding in ["jpeg", "png"]:
                    # 压缩格式，需要解码
                    byte_data = np_array.tobytes()
                    image = cv2.imdecode(np.frombuffer(byte_data, np.uint8), cv2.IMREAD_COLOR)

                # 显示图像
                if image is not None:
                    cv2.imshow("Received Image", image)
                    cv2.waitKey(1)


if __name__ == "__main__":
    main()
