import { PrivyProvider } from "@privy-io/react-auth";

const privyAppId = import.meta.env.VITE_PRIVY_APP_ID as string;
const privyClientId = import.meta.env.VITE_PRIVY_CLIENT_ID as string;

const config: any = {
  // Display email and wallet as login methods
  appearance: {
    loginMethods: ["email", "wallet"],
    theme: "light",
    accentColor: "#676FFF",
    logo: "https://your-logo-url",
  },
  // Create embedded wallets for users who don't have a wallet
  embeddedWallets: {
    createOnLogin: "users-without-wallets",
  },
};

type PrivyProviderProps = {
  children: React.ReactNode;
};

const AddPrivyProvider = ({ children }: PrivyProviderProps) => (
  <PrivyProvider appId={privyAppId} clientId={privyClientId} config={config}>
    {children}
  </PrivyProvider>
);

export default AddPrivyProvider;
