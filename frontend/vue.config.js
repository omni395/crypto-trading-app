module.exports = {
  devServer: {
    port: 8080
  },
  publicPath: '/',
  outputDir: '../static',
  assetsDir: '',
  indexPath: 'index.html', // Указываем, что index.html находится в frontend/
  chainWebpack: (config) => {
    config.plugin('define').tap((args) => {
      args[0]['__VUE_PROD_HYDRATION_MISMATCH_DETAILS__'] = false;
      return args;
    });
  }
};