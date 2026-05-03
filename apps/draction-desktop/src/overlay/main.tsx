import React from "react";
import ReactDOM from "react-dom/client";
import OverlayApp from "./App";
import { ToastProvider } from "../components/ui/Toast";
import { applyThemeToDocument, useUiStore } from "../stores/uiStore";
import "../styles.css";

applyThemeToDocument(useUiStore.getState().theme);

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ToastProvider>
      <OverlayApp />
    </ToastProvider>
  </React.StrictMode>,
);
