import { Button } from "antd";
import React from "react";
import { useNavigate } from "react-router-dom";

export const AboutPage: React.FC = () => {
  const navigate = useNavigate();

  return (
    <>
      <h1>ABOUT PAGE</h1>
      <br />
      <Button type="primary" onClick={() => navigate("/")}>
        Go home
      </Button>
    </>
  );
};
