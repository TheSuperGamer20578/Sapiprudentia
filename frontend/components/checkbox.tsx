import React, {useState} from "react";
import {ReactNode} from "react";
import ConfettiExplosion from "react-confetti-explosion";
import useSound from "use-sound";

interface Props {
    checked: boolean;
    setChecked: (checked: boolean) => void;
    children?: ReactNode;
}

export default function Checkbox({checked, setChecked, children}: Props) {
    const [exploding, setExploding] = useState(false);
    const [play] = useSound("/static/complete.wav");

    return <>
        {exploding && <ConfettiExplosion />}
        <label>
            <input
                type="checkbox"
                checked={checked}
                onChange={() => {
                    setExploding(!checked);
                    if (!checked) {
                        play();
                    }
                    setChecked(!checked);
                }}
            />
            {children}
        </label>
    </>;
}
