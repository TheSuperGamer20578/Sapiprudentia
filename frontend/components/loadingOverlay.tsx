import React from "react";
import Overlay from "./overlay";
import Loader from "./loader";
import {useNavigation} from "react-router-dom";

export default function LoadingOverlay({force}: { force?: boolean }) {
    if (force === undefined) {
        const navigation = useNavigation();

        return navigation.state === "loading" && <Overlay><Loader /></Overlay>;
    }
    return force && <Overlay><Loader /></Overlay>;
}
