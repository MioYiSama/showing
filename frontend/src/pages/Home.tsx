import { UserOutlined } from "@ant-design/icons";
import { Button } from "antd";
import { useRef } from "preact/hooks";

export default () => {
  return (
    <>
      <UserOutlined />
      <Button type="primary">Test</Button>
    </>
  );
};
