import { useState } from "react";
import TypeParameters from "../api/Run/TypeParameters";
import PostParameters from "../api/Run/PostParameters";

const SimulationParameters = () => {

    const [name, setName] = useState("");
    const [numAtoms, setNumAtoms] = useState("1");
    const [boxlength, setBoxlength] = useState("0");
    const [step, setStep] = useState("0");
    const [temperature, setTemperature] = useState("0");

    const SubmitParameters = () => {
        const sendParameters: TypeParameters = {
            nAtoms: Number(numAtoms),
            boxlength: Number(boxlength),
            step: Number(step),
            temperature: Number(temperature),
        }
        PostParameters(name, sendParameters)
    }
    const validNumberTitle = "Insert a valid number as decimal or in scientific notation (e.g. 1.2e-3)"

    return (
        <>
            <section className="border-solid border-2 rounded-lg border-sky-600 p-4">
                <form onSubmit={SubmitParameters}>
                    <h1 className="text-2xl font-bold">
                        Parameters
                    </h1>
                    <div className="mt-4">
                        <label>
                            Name of the run:
                            <input
                                type="text"
                                id="runName"
                                className="border rounded w-full py-2 px-3 mt-2 invalid:border-red-600"
                                required={true}
                                value={name}
                                placeholder="Name of the run"
                                onChange={(e) => setName(e.target.value)}
                                pattern="[a-zA-Z0-9]+"
                                title="Insert only letters or numbers"
                            />
                        </label>
                    </div>
                    <div className="mt-4">
                        <label>
                            Number of atoms for the simulation:
                            <input
                                type="text"
                                min={1}
                                id="nAtoms"
                                name=""
                                className="border rounded w-full py-2 px-3 mt-2 invalid:border-red-600"
                                required={true}
                                value={numAtoms}
                                onChange={(e) => setNumAtoms(e.target.value)}
                                pattern="\d+"
                                onInvalid={(e) => {console.log(e);
                                }}
                                title="Insert an integer number greater than zero"
                            />
                        </label>
                    </div>
                    <div className="mt-4">
                        <label>
                            Length of the box:
                            <input 
                                type="text"
                                min={0}
                                id="boxlenght"
                                name=""
                                className="border rounded w-full py-2 px-3 mt-2 invalid:border-red-600"
                                required={true}
                                value={boxlength}
                                onChange={(e) => setBoxlength(e.target.value)}
                                pattern="\d*\.?\d*(?:e(?:|-|\+)\d+)?"
                                title={validNumberTitle}
                            />
                        </label>
                    </div>
                    <div className="mt-4">
                        <label>
                            Step of the simulation:
                            <input 
                                type="text"
                                min={0}
                                id="nAtoms"
                                name=""
                                className="border rounded w-full py-2 px-3 mt-2 invalid:border-red-600"
                                required={true}
                                value={step}
                                onChange={(e) => setStep(e.target.value)}
                                pattern="\d*\.?\d*(?:e(?:|-|\+)\d+)?"
                                title={validNumberTitle}
                            />
                        </label>
                    </div>
                    <div className="mt-4">
                        <label>
                            Temperature of the ensemble:
                            <input 
                                type="number"
                                min={0}
                                id="temperature"
                                name=""
                                className="border rounded w-full py-2 px-3 mt-2 invalid:border-red-600"
                                required={true}
                                value={temperature}
                                onChange={(e) => setTemperature(e.target.value)}
                                pattern="\d*\.?\d*(?:e(?:|-|\+)\d+)?"
                                title={validNumberTitle}
                            />
                        </label>
                    </div>
                    <div className="flex pt-2 justify-end">
                        <button className="bg-indigo-600 hover:bg-blue-600 text-indigo-50 py-2 px-4 rounded-lg justify-self-end">Run</button>
                    </div>
                </form>
            </section>
       </>
    );
}
export default SimulationParameters
