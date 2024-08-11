import React, { useState } from 'react';

function App() {
  const [message, setMessage] = useState('');

  const handleButtonClick = async (action) => {
    try {
      const response = await fetch(`http://localhost:8080/${action}`);
      const result = await response.json();
      setMessage(result.message); // 更新页面内容
    } catch (error) {
      console.error('Error:', error);
      setMessage('请求失败');
    }
  };


  

  return (
    <div>
      <h1>Rust Vite Demo</h1>
      <button onClick={() => handleButtonClick('query')}>查询</button>
      <button onClick={() => handleButtonClick('action1')}>执行操作1</button>
      <button onClick={() => handleButtonClick('action2')}>执行操作2</button>
      <p>{message}</p> {/* 在页面上显示返回的消息 */}
    </div>
  );
}

export default App;
