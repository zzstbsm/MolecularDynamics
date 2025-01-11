import { Route, createBrowserRouter, createRoutesFromElements, RouterProvider } from "react-router-dom";
import MainLayout from "./layout/MainLayout"
import Homepage from "./components/Homepage";
import NotFoundPage from "./components/NotFoundPage";

const App = () => {

    const contentWidth = "w-4/6 m-auto"

    const router = createBrowserRouter(
        createRoutesFromElements(
            <Route path='/' element={<MainLayout width={contentWidth} />} >
                <Route index element={<Homepage width={contentWidth} />} />
                <Route path="*" element={<NotFoundPage />} />
            </Route>
        )
    );

    return (
        <RouterProvider router={router} />
    );

}

export default App
