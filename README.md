# ToFixed Attribute Macro for Rust

Bu proje, sabit nokta (fixed-point) sayıları dönüştürmek için bir Rust procedural macro (`to_fixed`) sağlar. Bu makro, belirli bir sabit diziyi alır ve her elemanı, sabit nokta biçimine dönüştürerek `fixed::types` kütüphanesinin türlerini kullanarak sabitlere dönüştürür.

## Özellikler

- `U{int_bits}F{frac_bits}` gibi sabit nokta türleriyle çalışır.
- Sabit bir diziye uygulandığında, dizideki her eleman sabit nokta türüne dönüştürülür.
- Şu anda yalnızca `f32` türündeki değerlerle çalışmaktadır.
  
## Kurulum

## Kullanım
```rust
use fixed::types::U2F14;

#[to_fixed]
const SIN_TABLE: [U2F14; 256] = [
    0.0, 0.1, 0.2, 0.3, // ... devamı
];

fn main() {
    println!("{:?}", SIN_TABLE);
}
```
## Katkı Sağlamak

Projeye katkı sağlamak isteyen herkes memnuniyetle kabul edilmektedir. Eğer eksik veya geliştirilmesi gereken bir şey olduğunu düşünüyorsanız, bir pull request göndererek veya issue açarak katkı sağlayabilirsiniz.

### Katkı Kuralları

- Bu projede yapılan değişiklikler, proje kodunu daha stabil ve kullanışlı hale getirmeyi amaçlamalıdır.
- Yeni özellikler veya düzeltmeler için, mümkünse testler eklemeyi unutmayın.
- PR göndermeden önce lütfen açık bir şekilde neyi değiştirdiğinizi ve neden değiştirdiğinizi açıklayan bir açıklama ekleyin.

### Gelecek Planları

Projenin zamanla eksiklerinin giderilmesi ve yeni özelliklerin eklenmesi beklenmektedir. Bu özellikler şunları içerebilir:
- Diğer sabit nokta türlerinin desteklenmesi.
- Performans iyileştirmeleri.
- Daha fazla test ve hata ayıklama.

### Lisans

Bu proje MIT Lisansı altında lisanslanmıştır. Daha fazla bilgi için LICENSE dosyasını inceleyebilirsiniz.
