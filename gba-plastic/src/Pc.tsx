import { useContext, useState } from 'react';
import { GbaContext } from './Gba';
import useAnimationFrame from './useAnimationFrame';

const Pc = () => {
    const [pc, setPc] = useState<number | undefined>();
    const gba = useContext(GbaContext);

    useAnimationFrame(() => {
        const pc = gba?.inspect_cpu().pc;
        setPc(pc);
    });

    const pcString = pc === undefined ? 'undefined' : `0x${pc.toString(16)}`;

    return <div>PC: {pcString}</div>;
};

export default Pc;
