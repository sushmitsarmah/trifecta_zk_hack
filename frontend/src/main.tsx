import { StrictMode } from "react";
import ReactDOM from "react-dom/client";

// providers
import AddPrivyProvider from "./web3/wallets/privy";
import {
  Provider as TanStackQueryProvider
} from "./integrations/tanstack-query/root-provider";

import "./styles.css";
import App from "./App.tsx";

import reportWebVitals from "./reportWebVitals.ts";

const rootElement = document.getElementById("app");
if (rootElement && !rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement);
  root.render(
    <StrictMode>
      <AddPrivyProvider>
        <TanStackQueryProvider>
          <App />
        </TanStackQueryProvider>
      </AddPrivyProvider>
    </StrictMode>,
  );
}

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
