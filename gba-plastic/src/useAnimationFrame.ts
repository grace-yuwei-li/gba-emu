import { useCallback, useEffect, useRef } from 'react';

const useAnimationFrame = (callback: (deltaTime: number) => void) => {
    const previousTimeStamp = useRef<number | undefined>(undefined);
    const animationFrameId = useRef<number | undefined>(undefined);

    const frame = useCallback(
        (timeStamp: number) => {
            if (previousTimeStamp.current === undefined) {
                previousTimeStamp.current = timeStamp;
            }
            const deltaTime = timeStamp - previousTimeStamp.current;

            if (deltaTime !== 0) {
                callback(deltaTime);
            }

            previousTimeStamp.current = timeStamp;
            animationFrameId.current = window.requestAnimationFrame(frame);
        },
        [callback]
    );

    useEffect(() => {
        animationFrameId.current = window.requestAnimationFrame(frame);
        return () => {
            if (animationFrameId.current !== undefined) {
                window.cancelAnimationFrame(animationFrameId.current);
            }
        };
    }, [callback]);
};

export default useAnimationFrame;
