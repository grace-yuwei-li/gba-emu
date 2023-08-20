import './App.css';
import MemoryView from './MemoryView';
import Pc from './Pc';
import Ppu from './debugger/Ppu';

function App() {
    return (
        <>
            <Pc />
            <Ppu />
            <MemoryView />
        </>
    );
}

export default App;
