import React, {ReactNode, useEffect, useState} from "react";
import Loader from "./loader";

interface Props {
    effect: () => Promise<ReactNode>,
}

export default function EffectValue({effect}: Props) {
    const [value, setValue] = useState<ReactNode>(null);
    useEffect(() => {
        effect().then(setValue);
    }, []);
    if (value === null) {
        return <Loader />;
    }
    return value;
}
