import initWasm, { GbaCore } from '../pkg/gba_core';
import { ReactNode, createContext, useEffect, useState } from 'react';

async function initGba() {
    await initWasm();
}

export const GbaContext = createContext<GbaCore | undefined>(undefined);

interface GbaProviderProps {
    children: ReactNode
}

const GbaProvider = ({ children }: GbaProviderProps) => {
    const [gba, setGba] = useState<GbaCore | undefined>(undefined);

    useEffect(() => {
        initGba().then(() => {
            setGba(new GbaCore()); 
        });
    }, []);

    return (
        <GbaContext.Provider value={gba}>
            {children}
        </GbaContext.Provider>
    )
};

export default GbaProvider
