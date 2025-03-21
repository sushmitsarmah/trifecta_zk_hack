import { usePrivy } from "@privy-io/react-auth";
import router from "./Router";

const App = () => {
  const { ready } = usePrivy();

  if (!ready) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      {router}
    </div>
  );
}

export default App;
