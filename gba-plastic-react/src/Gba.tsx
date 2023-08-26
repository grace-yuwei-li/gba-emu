import initWasm, { GbaCore } from '../pkg/gba_core';
import {
    ReactNode,
    createContext,
    useContext,
    useEffect,
    useState,
} from 'react';
import useAnimationFrame from './useAnimationFrame';

async function initGba() {
    await initWasm();
}

export type CustomEventListener = (e: CustomEvent) => void;

const GbaContext = createContext<GbaCore | undefined>(undefined);

export const useGba = () => {
    const gba = useContext(GbaContext);
    return gba;
};

interface GbaProviderProps {
    children: ReactNode;
}

const GbaProvider = ({ children }: GbaProviderProps) => {
    const [gba, setGba] = useState<GbaCore | undefined>(undefined);
    useAnimationFrame(() => {
        const start = performance.now();
        for (let i = 0; i < 1; i++) {
            gba?.tick();
        }
        const elapsed = performance.now() - start;
    });

    useEffect(() => {
        initGba().then(() => {
            const gba = new GbaCore();
            gba.load_test_rom();
            gba.skip_bios();
            setGba(gba);
        });
    }, []);

    return <GbaContext.Provider value={gba}>{children}</GbaContext.Provider>;
};

export default GbaProvider;
