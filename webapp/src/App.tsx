import Foo from "./routes/Foo";
import Home from "./routes/Home";
import Login from "./routes/Login";
import { RouterProvider } from "react-router";
import { createBrowserRouter } from "react-router-dom";
import styled from "styled-components";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/login",
    element: <Login />,
  },
  {
    path: "/foo/:id",
    element: <Foo />,
  },
]);

const Container = styled.div`
  width: 100%;
  height: 100vh;
`;

const App: React.FC = () => {
  return (
    <Container>
      <RouterProvider router={router} />
    </Container>
  );
};

export default App;
