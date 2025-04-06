module.exports = {
  devServer: {
    port: 8080,
    hot: true, // Включаем HMR
    liveReload: true // Включаем live-reload
  },
  publicPath: '/',
  outputDir: '../static',
  assetsDir: '',
  indexPath: 'index.html',
  pages: {
    index: {
      entry: 'src/main.js',
      template: 'public/index.html',
      filename: 'index.html'
    }
  },
  chainWebpack: (config) => {
    config.plugin('define').tap((args) => {
      args[0]['__VUE_PROD_HYDRATION_MISMATCH_DETAILS__'] = false;
      return args;
    });
  }
};