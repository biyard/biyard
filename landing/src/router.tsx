import { createBrowserRouter } from "react-router";
import { IndexPage } from "./pages";
import { SignInPage } from "./pages/signin";
import { WalletPage } from "./pages/wallet";
import { DAOPage } from "./pages/dao";

export const routes = createBrowserRouter(
  [
    {
      id: "index-page",
      path: "/",
      index: true,
      Component: IndexPage,
    },
    {
      path: "/signin",
      Component: SignInPage,
    },
    {
      path: "/signup",
      Component: SignInPage,
    },
    {
      path: "/wallet",
      Component: WalletPage,
    },
    {
      path: "/dao",
      Component: DAOPage,
    },
  ],
  {
    basename: import.meta.env.BASE_URL,
  }
);
