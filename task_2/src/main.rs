fn main() {}

/// Умная розетка.
struct _SmartSocket {
    /// Текстовое описание розетки.
    description: String,
    /// Флаг, указывающий, включена ли розетка.
    is_on: bool,
    /// Текущая потребляемая мощность (в ваттах).
    power_consumption: f32,
}

impl _SmartSocket {
    /// Включает розетку.
    fn _turn_on(&mut self) {
        todo!();
    }

    /// Выключает розетку.
    fn _turn_off(&mut self) {
        todo!();
    }

    /// Возвращает описание розетки.
    fn _get_description(&self) -> &str {
        &self.description
    }

    /// Возвращает данные о текущей потребляемой мощности.
    fn _get_power_consumption(&self) -> f32 {
        self.power_consumption
    }
}

/// Термометр.
struct _Thermometer {
    /// Текущая температура (в градусах Цельсия).
    temperature: f32,
}

impl _Thermometer {
    /// Возвращает текущую температуру.
    fn _get_temperature(&self) -> f32 {
        todo!();
    }
}
