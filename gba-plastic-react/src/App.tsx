import './App.css';
import MemoryView from './MemoryView';
import Pc from './Pc';
import Instructions from './debugger/Instructions';
import Ppu from './debugger/Ppu';

function App() {
    return (
        <>
            <Instructions />
            <Pc />
            <Ppu />
            <MemoryView />
        </>
    );
}

export default App;
