import TypeParameters from "./TypeParameters";

const PostParameters = async (name: string, param: TypeParameters) => {
    await fetch(`api/run/${name}`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(param),
    });
    return;
};
export default PostParameters;
