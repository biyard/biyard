import { createBrowserRouter } from "react-router";
import { IndexPage } from "./pages";

export const routes = createBrowserRouter(
  [
    {
      id: "index-page",
      path: "/",
      index: true,
      Component: IndexPage,
    },
  ],
  {
    basename: import.meta.env.BASE_URL,
  },
);
