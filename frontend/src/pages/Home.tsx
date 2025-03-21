
import { usePrivy } from "@privy-io/react-auth";
import { Button } from "../components/ui/button";

function Home() {
  const { ready, authenticated, user, login, logout } = usePrivy();

  if (!ready) {
    return <div>Loading...</div>;
  }

  return (
    <div className="text-center">
      <header className="min-h-screen flex flex-col items-center justify-center bg-[#282c34] text-white text-[calc(10px+2vmin)]">
        <img
          src={"logo.svg"}
          className="h-[40vmin] pointer-events-none animate-[spin_20s_linear_infinite]"
          alt="logo"
        />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="text-[#61dafb] hover:underline"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <a
          className="text-[#61dafb] hover:underline"
          href="https://tanstack.com"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn TanStack
        </a>
        {ready && authenticated ? (
          <div>
            <textarea
              readOnly
              value={JSON.stringify(user, null, 2)}
              style={{ width: "600px", height: "250px", borderRadius: "6px" }}
            />
            <br />
            <Button onClick={logout}>Log Out</Button>
          </div>
        ) : (
          <Button onClick={login}>Login</Button>
        )}
      </header>
    </div>
  );
}

export default Home;
