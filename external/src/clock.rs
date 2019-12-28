use crate::obj::AsObject;

/*
EXTERN t_clock *clock_new(void *owner, t_method fn);
EXTERN void clock_set(t_clock *x, double systime);
EXTERN void clock_delay(t_clock *x, double delaytime);
EXTERN void clock_unset(t_clock *x);
EXTERN void clock_setunit(t_clock *x, double timeunit, int sampflag);
EXTERN double clock_getlogicaltime(void);
EXTERN double clock_getsystime(void); /* OBSOLETE; use clock_getlogicaltime() */
EXTERN double clock_gettimesince(double prevsystime);
EXTERN double clock_gettimesincewithunits(double prevsystime,
    double units, int sampflag);
EXTERN double clock_getsystimeafter(double delaytime);
EXTERN void clock_free(t_clock *x);
*/

//TODO figure out units and provide those as enum

pub struct Clock(*mut pd_sys::_clock);

impl Clock {
    pub fn logical_time() -> f64 {
        unsafe { pd_sys::clock_getlogicaltime() }
    }

    pub fn sys_time_after(delaytime: f64) -> f64 {
        unsafe { pd_sys::clock_getsystimeafter(delaytime) }
    }

    pub fn time_since(prev: f64) -> f64 {
        unsafe { pd_sys::clock_gettimesince(prev) }
    }

    pub fn new(owner: &mut dyn AsObject, method: crate::method::PdMethod) -> Self {
        unsafe {
            Self(pd_sys::clock_new(
                std::mem::transmute::<_, _>(owner.as_obj()),
                Some(method),
            ))
        }
    }

    /// call back at a delayed time
    pub fn delay(&mut self, delay: f64) {
        unsafe {
            pd_sys::clock_delay(self.0, delay);
        }
    }

    /// call back at an absolute time
    pub fn set(&mut self, absolute: f64) {
        unsafe {
            pd_sys::clock_set(self.0, absolute);
        }
    }

    /// cancel the clock (unset)
    pub fn cancel(&mut self) {
        unsafe {
            pd_sys::clock_unset(self.0);
        }
    }
}

impl Drop for Clock {
    fn drop(&mut self) {
        unsafe {
            pd_sys::clock_free(self.0);
        }
    }
}
