import SimulationParameters from "./SimulationParameters"

const Homepage = ({
    width = "m-auto"
}) => {
    return <>
        <section className={`${width} mt-4`}>
            <SimulationParameters />
        </section>
    </>
}
export default Homepage
