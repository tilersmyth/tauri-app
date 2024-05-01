import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import { ConfigProvider } from "antd";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ConfigProvider
      theme={{
        components: {
          Button: {
            colorPrimary: "#00b96b",
            padding: 20,
            algorithm: true, // Enable algorithm
          },
          Input: {
            colorPrimary: "#eb2f96",
            algorithm: true, // Enable algorithm
          },
        },
      }}
    >
      <App />
    </ConfigProvider>
  </React.StrictMode>
);
