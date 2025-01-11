const NavBar = ({
    width = "m-auto",
    title = "",
}) => {

    return (
        <section className='bg-indigo-700'>
            <div className={`${width} py-4`}>
                <h1 className='text-2xl text-indigo-200 text-start'>
                    {title}
                </h1>
            </div>
        </section>
    );
}
export default NavBar
