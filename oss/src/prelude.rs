/*
   实现一个new的入口,new的一些参数
   以及对应要实现的接口定义
*/

pub trait Oss {
    //签名是 pub_object(key:文件名，content：文件类型，是字节形式)，返回值:是一个err
    //签名是 pub_object(key:文件名，content：io.Reader)，返回值:是一个err

    fn pub_object(
        &self,
        key: &str,
        content: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    fn pub_object_with_reader(
        &self,
        key: &str,
        content: &mut dyn std::io::Read,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
