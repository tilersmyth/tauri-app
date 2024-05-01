import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "antd";
import React from "react";
import { useNavigate } from "react-router-dom";
import { Cat } from "../bindings/Cat";

interface Response<T> {
  result: {
    data: T;
  };
  error: string;
}

export const HomePage: React.FC = () => {
  const navigate = useNavigate();

  const test = async () => {
    try {
      const res = await invoke<Response<Cat>>("get_cat", {
        params: { id: "1" },
      });

      console.log(res);
    } catch (error) {
      console.log("ERROR: ", error);
    }
  };

  return (
    <>
      <h1>HOME PAGE</h1>
      <br />
      <br />
      <Button onClick={test}>Test!</Button>
      <br />
      <br />
      <Button type="primary" onClick={() => navigate("/about")}>
        Go to about
      </Button>
      <br />
    </>
  );
};
