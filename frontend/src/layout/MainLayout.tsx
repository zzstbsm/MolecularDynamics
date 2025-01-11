import { Outlet } from "react-router-dom"
import NavBar from "../components/Navbar"

const MainLayout = ({
    width = "m-auto"
}) => {

    return <>
        <NavBar
            width={width}
            title="Molecular Dynamics"
        />
        <Outlet />
    </>
}
export default MainLayout
