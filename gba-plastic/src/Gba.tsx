import initWasm, { GbaCore } from '../pkg/gba_core';
import { ReactNode, createContext, useEffect, useState } from 'react';
import useAnimationFrame from './useAnimationFrame';

async function initGba() {
    await initWasm();
}

export type CustomEventListener = (e: CustomEvent) => void;

export const GbaContext = createContext<GbaCore | undefined>(undefined);

interface GbaProviderProps {
    children: ReactNode;
}

const GbaProvider = ({ children }: GbaProviderProps) => {
    const [gba, setGba] = useState<GbaCore | undefined>(undefined);
    useAnimationFrame(() => {
        const start = performance.now();
        for (let i = 0; i < 10000; i++) {
            gba?.tick();
        }
        const elapsed = performance.now() - start;

        console.log(elapsed);
    });

    useEffect(() => {
        initGba().then(() => {
            const gba = new GbaCore();
            gba.load_panda();
            gba.skip_bios();
            setGba(gba);
        });
    }, []);

    return <GbaContext.Provider value={gba}>{children}</GbaContext.Provider>;
};

export default GbaProvider;
