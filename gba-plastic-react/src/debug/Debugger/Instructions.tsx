import { CSSProperties } from 'react';
import { useGba } from '../../Gba';
import { FixedSizeList } from 'react-window';

interface ItemData {
    getAddress: (index: number) => number;
    getInstruction: unknown;
}

interface RowProps {
    index: number;
    data: ItemData;
    style: CSSProperties;
}

const Row = ({ index, data, style }: RowProps) => {
    const { getAddress, getInstruction } = data;

    const address = getAddress(index);
    const hexAddr = `0x${address.toString(16)}`;

    return <div style={style}>{hexAddr}</div>;
};

const Instructions = () => {
    const gba = useGba();

    const getAddress = (index: number): number => {
        return index * 4;
    };

    const getInstruction = (index: number): number | undefined => {
        return gba?.read_address(getAddress(index));
    };

    const itemData = {
        getAddress,
        getInstruction,
    };

    return (
        <FixedSizeList
            height={500}
            itemCount={4294967296 / 4}
            itemSize={30}
            width={300}
            overscanCount={50}
            itemData={itemData}
        >
            {Row}
        </FixedSizeList>
    );
};

export default Instructions;
