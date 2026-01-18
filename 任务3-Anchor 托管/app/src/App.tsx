import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { WalletContextProvider } from './components/WalletContextProvider';
import { MakeForm } from './components/forms/MakeForm';
import { TakeForm } from './components/forms/TakeForm';
import { RefundForm } from './components/forms/RefundForm';
import './App.css';

function App() {
  return (
    <WalletContextProvider>
      <div className="container">
        <header className="header">
          <h1>Anchor 担保交易</h1>
          <WalletMultiButton />
        </header>
        <main>
          <div className="card">
            <h2>创建订单 (Make)</h2>
            <p>初始化一个新的担保交易订单。</p>
            <MakeForm />
          </div>

          <div className="card">
            <h2>完成订单 (Take)</h2>
            <p>履行并通过一个现有的订单。</p>
            <TakeForm />
          </div>

          <div className="card">
            <h2>退款 (Refund)</h2>
            <p>取消并收回你创建的订单。</p>
            <RefundForm />
          </div>
        </main>
      </div>
    </WalletContextProvider>
  );
}

export default App;
