import { useRef, useState } from 'react';
import { useGba } from '../Gba';
import useAnimationFrame from '../useAnimationFrame';

const Ppu = () => {
    const [bgMode, setBgMode] = useState<number | undefined>(undefined);
    const gba = useGba();
    const ref = useRef<HTMLCanvasElement>(null);

    useAnimationFrame(() => {
        const details = gba?.inspect_ppu();
        setBgMode(details?.bg_mode);

        const canvas_data = details?.screen();
        if (canvas_data) {
            const imageData = new ImageData(canvas_data, 240);
            createImageBitmap(imageData, {
                resizeWidth: 240 * 3,
                resizeHeight: 160 * 3,
                resizeQuality: 'pixelated',
            }).then((bitmap) => {
                if (ref.current) {
                    const ctx = ref.current.getContext('2d');
                    ctx?.drawImage(bitmap, 0, 0);
                }
            });
        }
    });

    return (
        <>
            <div>BG Mode: {bgMode}</div>
            <canvas
                ref={ref}
                style={{ imageRendering: 'pixelated' }}
                width={240 * 3}
                height={160 * 3}
            ></canvas>
        </>
    );
};

export default Ppu;
