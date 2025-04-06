module.exports = {
  devServer: {
    port: 8080
  },
  publicPath: '/',
  chainWebpack: (config) => {
    config.plugin('define').tap((args) => {
      args[0]['__VUE_PROD_HYDRATION_MISMATCH_DETAILS__'] = false;
      return args;
    });
  }
};