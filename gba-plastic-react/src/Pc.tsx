import { useState } from 'react';
import useAnimationFrame from './useAnimationFrame';
import { useGba } from './Gba';

const Pc = () => {
    const [pc, setPc] = useState<number | undefined>();
    const gba = useGba();

    useAnimationFrame(() => {
        const pc = gba?.inspect_cpu().pc;
        setPc(pc);
    });

    const pcString = pc === undefined ? 'undefined' : `0x${pc.toString(16)}`;

    return <div>PC: {pcString}</div>;
};

export default Pc;
