import { createBrowserRouter } from "react-router-dom";
import { RouterProvider } from "react-router";

const router = createBrowserRouter([
  {
    path: "/",
    element: <p>Hello world!</p>,
  },
]);

const App: React.FC = () => {
  return (
    <>
      <RouterProvider router={router} />
    </>
  );
};

export default App;
