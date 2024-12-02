import React from "react";
import ReactDOM from "react-dom/client";
import { LocoSplash } from "./LocoSplash";
import { Layout } from "./components/Layout";

import "./index.css";

const root = document.getElementById("root");

if (!root) {
  throw new Error("No root element found");
}

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <Layout>
      <LocoSplash />
    </Layout>
  </React.StrictMode>,
);
