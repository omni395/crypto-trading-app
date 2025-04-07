// Полифилл для process, чтобы избежать ошибок в браузере
const process = {
    env: {
      NODE_ENV: 'development',
      VUE_APP_ENV: 'browser',
    },
  };
  
  export default process;