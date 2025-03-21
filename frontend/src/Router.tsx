import {
  Outlet,
  RouterProvider,
  createRootRoute,
  createRoute,
  createRouter,
} from "@tanstack/react-router";

import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import DemoTanstackQuery from "./routes/demo.tanstack-query";

// import Header from "./components/Header";
import Navbar from "./components/sections/navbar/default";
import Home from "./pages/Home";

import TanstackQueryLayout from "./integrations/tanstack-query/layout";
import * as TanstackQuery from "./integrations/tanstack-query/root-provider";

const rootRoute = createRootRoute({
  component: () => (
    <>
      {/* <Header /> */}
      <Navbar />
      <Outlet />
      <TanStackRouterDevtools />

      <TanstackQueryLayout />
    </>
  ),
});

// define all routes
const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/",
  component: Home,
});

// combine all routes
const routeTree = rootRoute.addChildren([
  indexRoute,
  DemoTanstackQuery(rootRoute),
]);

// create router
const router = createRouter({
  routeTree,
  context: {
    ...TanstackQuery.getContext(),
  },
  defaultPreload: "intent",
  scrollRestoration: true,
  defaultStructuralSharing: true,
  defaultPreloadStaleTime: 0,
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

export default <RouterProvider router={router} />;