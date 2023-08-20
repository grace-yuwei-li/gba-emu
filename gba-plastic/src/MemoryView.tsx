import { useContext, useRef, useState } from 'react';
import { GbaContext } from './Gba';
import useAnimationFrame from './useAnimationFrame';
import { to_canvas_binary_data } from '../pkg/gba_core';

const MemoryView = () => {
    const ref = useRef<HTMLCanvasElement>(null);

    const gba = useContext(GbaContext);
    useAnimationFrame((deltaTime: number) => {
        if (deltaTime > 8) {
            //console.warn('frame took too long:', deltaTime);
        }
        if (!gba) return;

        const vram = gba.inspect_memory().vram;
        const data = to_canvas_binary_data(vram);

        const imageData = new ImageData(data, 1536);
        createImageBitmap(imageData).then((bitmap) => {
            if (ref.current) {
                const ctx = ref.current.getContext('2d');
                ctx?.drawImage(bitmap, 0, 0);
            }
        });
    });

    return (
        <>
            <div>VRAM:</div>
            <canvas
                ref={ref}
                style={{ imageRendering: 'pixelated' }}
                width={1536}
                height={512}
            ></canvas>
        </>
    );
};

export default MemoryView;
