// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// THIS FILE IS AUTOGENERATED BY CARGO-LIBBPF-GEN!

pub use self::imp::*;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(clippy::absolute_paths)]
#[allow(clippy::transmute_ptr_to_ref)]
#[allow(clippy::upper_case_acronyms)]
#[warn(single_use_lifetimes)]
mod imp {
    #[allow(unused_imports)]
    use super::*;
    use libbpf_rs::libbpf_sys;
    use libbpf_rs::skel::OpenSkel;
    use libbpf_rs::skel::Skel;
    use libbpf_rs::skel::SkelBuilder;

    fn build_skel_config(
    ) -> libbpf_rs::Result<libbpf_rs::__internal_skel::ObjectSkeletonConfig<'static>> {
        let mut builder = libbpf_rs::__internal_skel::ObjectSkeletonConfigBuilder::new(DATA);
        builder
            .name("flames_bpf")
            .map("ring_buffer", false)
            .prog("handle_exec");

        builder.build()
    }

    #[derive(Default)]
    pub struct FlamesSkelBuilder {
        pub obj_builder: libbpf_rs::ObjectBuilder,
    }

    impl<'a> SkelBuilder<'a> for FlamesSkelBuilder {
        type Output = OpenFlamesSkel<'a>;
        fn open(self) -> libbpf_rs::Result<OpenFlamesSkel<'a>> {
            let opts = *self.obj_builder.opts();
            self.open_opts(opts)
        }

        fn open_opts(
            self,
            open_opts: libbpf_sys::bpf_object_open_opts,
        ) -> libbpf_rs::Result<OpenFlamesSkel<'a>> {
            let mut skel_config = build_skel_config()?;

            let ret =
                unsafe { libbpf_sys::bpf_object__open_skeleton(skel_config.get(), &open_opts) };
            if ret != 0 {
                return Err(libbpf_rs::Error::from_raw_os_error(-ret));
            }

            let obj = unsafe { libbpf_rs::OpenObject::from_ptr(skel_config.object_ptr())? };

            #[allow(unused_mut)]
            let mut skel = OpenFlamesSkel {
                obj,
                // SAFETY: Our `struct_ops` type contains only pointers,
                //         which are allowed to be NULL.
                // TODO: Generate and use a `Default` representation
                //       instead, to cut down on unsafe code.
                struct_ops: unsafe { std::mem::zeroed() },
                skel_config,
            };

            Ok(skel)
        }

        fn object_builder(&self) -> &libbpf_rs::ObjectBuilder {
            &self.obj_builder
        }
        fn object_builder_mut(&mut self) -> &mut libbpf_rs::ObjectBuilder {
            &mut self.obj_builder
        }
    }

    pub struct OpenFlamesMapsMut<'a> {
        inner: &'a mut libbpf_rs::OpenObject,
    }

    impl OpenFlamesMapsMut<'_> {
        pub fn ring_buffer(&mut self) -> &mut libbpf_rs::OpenMap {
            self.inner.map_mut("ring_buffer").unwrap()
        }
    }

    pub struct OpenFlamesMaps<'a> {
        inner: &'a libbpf_rs::OpenObject,
    }

    impl OpenFlamesMaps<'_> {
        pub fn ring_buffer(&self) -> &libbpf_rs::OpenMap {
            self.inner.map("ring_buffer").unwrap()
        }
    }

    pub struct OpenFlamesProgs<'a> {
        inner: &'a libbpf_rs::OpenObject,
    }

    impl OpenFlamesProgs<'_> {
        pub fn handle_exec(&self) -> &libbpf_rs::OpenProgram {
            self.inner.prog("handle_exec").unwrap()
        }
    }

    pub struct OpenFlamesProgsMut<'a> {
        inner: &'a mut libbpf_rs::OpenObject,
    }

    impl OpenFlamesProgsMut<'_> {
        pub fn handle_exec(&mut self) -> &mut libbpf_rs::OpenProgram {
            self.inner.prog_mut("handle_exec").unwrap()
        }
    }

    pub mod flames_types {
        #[allow(unused_imports)]
        use super::*;

        #[derive(Debug, Clone)]
        #[repr(C)]
        pub struct struct_ops {}

        impl struct_ops {}
    }

    pub struct OpenFlamesSkel<'a> {
        pub obj: libbpf_rs::OpenObject,
        pub struct_ops: flames_types::struct_ops,
        skel_config: libbpf_rs::__internal_skel::ObjectSkeletonConfig<'a>,
    }

    impl<'a> OpenSkel for OpenFlamesSkel<'a> {
        type Output = FlamesSkel<'a>;
        fn load(mut self) -> libbpf_rs::Result<FlamesSkel<'a>> {
            let ret = unsafe { libbpf_sys::bpf_object__load_skeleton(self.skel_config.get()) };
            if ret != 0 {
                return Err(libbpf_rs::Error::from_raw_os_error(-ret));
            }

            let obj = unsafe { libbpf_rs::Object::from_ptr(self.obj.take_ptr())? };

            Ok(FlamesSkel {
                obj,
                struct_ops: self.struct_ops,
                skel_config: self.skel_config,
                links: FlamesLinks::default(),
            })
        }

        fn open_object(&self) -> &libbpf_rs::OpenObject {
            &self.obj
        }

        fn open_object_mut(&mut self) -> &mut libbpf_rs::OpenObject {
            &mut self.obj
        }
    }
    impl OpenFlamesSkel<'_> {
        pub fn progs_mut(&mut self) -> OpenFlamesProgsMut<'_> {
            OpenFlamesProgsMut {
                inner: &mut self.obj,
            }
        }

        pub fn progs(&self) -> OpenFlamesProgs<'_> {
            OpenFlamesProgs { inner: &self.obj }
        }

        pub fn maps_mut(&mut self) -> OpenFlamesMapsMut<'_> {
            OpenFlamesMapsMut {
                inner: &mut self.obj,
            }
        }

        pub fn maps(&self) -> OpenFlamesMaps<'_> {
            OpenFlamesMaps { inner: &self.obj }
        }
    }

    pub struct FlamesMapsMut<'a> {
        inner: &'a mut libbpf_rs::Object,
    }

    impl FlamesMapsMut<'_> {
        pub fn ring_buffer(&mut self) -> &mut libbpf_rs::Map {
            self.inner.map_mut("ring_buffer").unwrap()
        }
    }

    pub struct FlamesMaps<'a> {
        inner: &'a libbpf_rs::Object,
    }

    impl FlamesMaps<'_> {
        pub fn ring_buffer(&self) -> &libbpf_rs::Map {
            self.inner.map("ring_buffer").unwrap()
        }
    }

    pub struct FlamesProgs<'a> {
        inner: &'a libbpf_rs::Object,
    }

    impl FlamesProgs<'_> {
        pub fn handle_exec(&self) -> &libbpf_rs::Program {
            self.inner.prog("handle_exec").unwrap()
        }
    }

    pub struct FlamesProgsMut<'a> {
        inner: &'a mut libbpf_rs::Object,
    }

    impl FlamesProgsMut<'_> {
        pub fn handle_exec(&mut self) -> &mut libbpf_rs::Program {
            self.inner.prog_mut("handle_exec").unwrap()
        }
    }

    #[derive(Default)]
    pub struct FlamesLinks {
        pub handle_exec: Option<libbpf_rs::Link>,
    }

    pub struct FlamesSkel<'a> {
        pub obj: libbpf_rs::Object,
        struct_ops: flames_types::struct_ops,
        skel_config: libbpf_rs::__internal_skel::ObjectSkeletonConfig<'a>,
        pub links: FlamesLinks,
    }

    unsafe impl Send for FlamesSkel<'_> {}
    unsafe impl Sync for FlamesSkel<'_> {}

    impl Skel for FlamesSkel<'_> {
        fn object(&self) -> &libbpf_rs::Object {
            &self.obj
        }

        fn object_mut(&mut self) -> &mut libbpf_rs::Object {
            &mut self.obj
        }

        fn attach(&mut self) -> libbpf_rs::Result<()> {
            let ret = unsafe { libbpf_sys::bpf_object__attach_skeleton(self.skel_config.get()) };
            if ret != 0 {
                return Err(libbpf_rs::Error::from_raw_os_error(-ret));
            }

            self.links = FlamesLinks {
                handle_exec: core::ptr::NonNull::new(self.skel_config.prog_link_ptr(0)?)
                    .map(|ptr| unsafe { libbpf_rs::Link::from_ptr(ptr) }),
            };

            Ok(())
        }
    }
    impl FlamesSkel<'_> {
        pub fn progs_mut(&mut self) -> FlamesProgsMut<'_> {
            FlamesProgsMut {
                inner: &mut self.obj,
            }
        }

        pub fn progs(&self) -> FlamesProgs<'_> {
            FlamesProgs { inner: &self.obj }
        }

        pub fn maps_mut(&mut self) -> FlamesMapsMut<'_> {
            FlamesMapsMut {
                inner: &mut self.obj,
            }
        }

        pub fn maps(&self) -> FlamesMaps<'_> {
            FlamesMaps { inner: &self.obj }
        }

        pub fn struct_ops_raw(&self) -> *const flames_types::struct_ops {
            &self.struct_ops
        }

        pub fn struct_ops(&self) -> &flames_types::struct_ops {
            &self.struct_ops
        }
    }

    const DATA: &[u8] = &[
        127, 69, 76, 70, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 247, 0, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0,
        64, 0, 9, 0, 1, 0, 0, 46, 115, 116, 114, 116, 97, 98, 0, 46, 115, 121, 109, 116, 97, 98, 0,
        116, 112, 47, 115, 99, 104, 101, 100, 47, 115, 99, 104, 101, 100, 95, 112, 114, 111, 99,
        101, 115, 115, 95, 101, 120, 101, 99, 0, 108, 105, 99, 101, 110, 115, 101, 0, 46, 109, 97,
        112, 115, 0, 102, 108, 97, 109, 101, 115, 46, 98, 112, 102, 46, 99, 0, 76, 66, 66, 48, 95,
        50, 0, 104, 97, 110, 100, 108, 101, 95, 101, 120, 101, 99, 0, 114, 105, 110, 103, 95, 98,
        117, 102, 102, 101, 114, 0, 76, 73, 67, 69, 78, 83, 69, 0, 46, 114, 101, 108, 116, 112, 47,
        115, 99, 104, 101, 100, 47, 115, 99, 104, 101, 100, 95, 112, 114, 111, 99, 101, 115, 115,
        95, 101, 120, 101, 99, 0, 46, 66, 84, 70, 0, 46, 66, 84, 70, 46, 101, 120, 116, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 4, 0,
        241, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 3, 0, 120, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 79, 0, 0, 0, 18, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 0, 0, 0, 0,
        0, 0, 0, 91, 0, 0, 0, 17, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 103, 0,
        0, 0, 17, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 24, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 2, 0, 0, 8, 0, 0, 0, 183, 3, 0, 0, 0, 0, 0, 0, 133, 0, 0,
        0, 131, 0, 0, 0, 191, 6, 0, 0, 0, 0, 0, 0, 21, 6, 8, 0, 0, 0, 0, 0, 133, 0, 0, 0, 7, 0, 0,
        0, 99, 6, 0, 0, 0, 0, 0, 0, 133, 0, 0, 0, 14, 0, 0, 0, 119, 0, 0, 0, 32, 0, 0, 0, 99, 6, 4,
        0, 0, 0, 0, 0, 191, 97, 0, 0, 0, 0, 0, 0, 183, 2, 0, 0, 0, 0, 0, 0, 133, 0, 0, 0, 132, 0,
        0, 0, 183, 0, 0, 0, 0, 0, 0, 0, 149, 0, 0, 0, 0, 0, 0, 0, 68, 117, 97, 108, 32, 66, 83, 68,
        47, 71, 80, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0, 159, 235, 1, 0, 24, 0, 0, 0, 0, 0, 0, 0, 60, 1, 0, 0, 60,
        1, 0, 0, 178, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 4, 0, 0,
        0, 32, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 27, 0, 0, 0, 5,
        0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 6, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 3, 0, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 2, 0, 0, 4, 16,
        0, 0, 0, 25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 0, 5, 0, 0, 0, 64, 0, 0, 0, 42, 0,
        0, 0, 0, 0, 0, 14, 7, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 10, 0, 0, 0, 54, 0, 0,
        0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 13, 2, 0, 0, 0, 89, 0, 0, 0, 9, 0, 0, 0,
        93, 0, 0, 0, 1, 0, 0, 12, 11, 0, 0, 0, 105, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 8, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 13, 0, 0, 0, 4, 0, 0, 0, 13, 0, 0, 0, 110, 0, 0, 0, 0, 0,
        0, 14, 14, 0, 0, 0, 1, 0, 0, 0, 136, 1, 0, 0, 1, 0, 0, 15, 13, 0, 0, 0, 15, 0, 0, 0, 0, 0,
        0, 0, 13, 0, 0, 0, 144, 1, 0, 0, 1, 0, 0, 15, 16, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 16, 0,
        0, 0, 0, 105, 110, 116, 0, 95, 95, 65, 82, 82, 65, 89, 95, 83, 73, 90, 69, 95, 84, 89, 80,
        69, 95, 95, 0, 116, 121, 112, 101, 0, 109, 97, 120, 95, 101, 110, 116, 114, 105, 101, 115,
        0, 114, 105, 110, 103, 95, 98, 117, 102, 102, 101, 114, 0, 116, 114, 97, 99, 101, 95, 101,
        118, 101, 110, 116, 95, 114, 97, 119, 95, 115, 99, 104, 101, 100, 95, 112, 114, 111, 99,
        101, 115, 115, 95, 101, 120, 101, 99, 0, 99, 116, 120, 0, 104, 97, 110, 100, 108, 101, 95,
        101, 120, 101, 99, 0, 99, 104, 97, 114, 0, 76, 73, 67, 69, 78, 83, 69, 0, 47, 114, 111,
        111, 116, 47, 102, 108, 97, 109, 101, 115, 47, 115, 114, 99, 47, 98, 112, 102, 47, 102,
        108, 97, 109, 101, 115, 46, 98, 112, 102, 46, 99, 0, 32, 32, 32, 32, 32, 32, 32, 32, 98,
        112, 102, 95, 114, 105, 110, 103, 98, 117, 102, 95, 114, 101, 115, 101, 114, 118, 101, 40,
        38, 114, 105, 110, 103, 95, 98, 117, 102, 102, 101, 114, 44, 32, 115, 105, 122, 101, 111,
        102, 40, 102, 108, 97, 109, 101, 115, 95, 115, 97, 109, 112, 108, 101, 41, 44, 32, 48, 41,
        59, 0, 32, 32, 32, 32, 105, 102, 32, 40, 115, 97, 109, 112, 108, 101, 32, 61, 61, 32, 48,
        41, 32, 123, 0, 32, 32, 32, 32, 115, 97, 109, 112, 108, 101, 45, 62, 115, 97, 109, 112,
        108, 101, 95, 105, 100, 32, 61, 32, 98, 112, 102, 95, 103, 101, 116, 95, 112, 114, 97, 110,
        100, 111, 109, 95, 117, 51, 50, 40, 41, 59, 0, 32, 32, 32, 32, 115, 97, 109, 112, 108, 101,
        45, 62, 112, 114, 111, 99, 101, 115, 115, 95, 105, 100, 32, 61, 32, 40, 95, 95, 117, 51,
        50, 41, 40, 98, 112, 102, 95, 103, 101, 116, 95, 99, 117, 114, 114, 101, 110, 116, 95, 112,
        105, 100, 95, 116, 103, 105, 100, 40, 41, 32, 62, 62, 32, 51, 50, 41, 59, 0, 32, 32, 32,
        32, 98, 112, 102, 95, 114, 105, 110, 103, 98, 117, 102, 95, 115, 117, 98, 109, 105, 116,
        40, 115, 97, 109, 112, 108, 101, 44, 0, 125, 0, 108, 105, 99, 101, 110, 115, 101, 0, 46,
        109, 97, 112, 115, 0, 116, 112, 47, 115, 99, 104, 101, 100, 47, 115, 99, 104, 101, 100, 95,
        112, 114, 111, 99, 101, 115, 115, 95, 101, 120, 101, 99, 0, 0, 0, 159, 235, 1, 0, 32, 0, 0,
        0, 0, 0, 0, 0, 20, 0, 0, 0, 20, 0, 0, 0, 156, 0, 0, 0, 176, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        0, 150, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 16, 0, 0, 0, 150, 1, 0, 0, 9, 0, 0,
        0, 0, 0, 0, 0, 118, 0, 0, 0, 152, 0, 0, 0, 9, 104, 0, 0, 48, 0, 0, 0, 118, 0, 0, 0, 221, 0,
        0, 0, 9, 108, 0, 0, 56, 0, 0, 0, 118, 0, 0, 0, 244, 0, 0, 0, 25, 128, 0, 0, 64, 0, 0, 0,
        118, 0, 0, 0, 244, 0, 0, 0, 23, 128, 0, 0, 72, 0, 0, 0, 118, 0, 0, 0, 35, 1, 0, 0, 34, 132,
        0, 0, 80, 0, 0, 0, 118, 0, 0, 0, 35, 1, 0, 0, 61, 132, 0, 0, 88, 0, 0, 0, 118, 0, 0, 0, 35,
        1, 0, 0, 24, 132, 0, 0, 96, 0, 0, 0, 118, 0, 0, 0, 103, 1, 0, 0, 5, 140, 0, 0, 120, 0, 0,
        0, 118, 0, 0, 0, 134, 1, 0, 0, 1, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 32, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 157, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 2, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 0, 0, 0, 0, 0, 0, 0, 168, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 17,
        0, 0, 0, 1, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 1, 0, 0, 0, 0, 0,
        0, 136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 45, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 2, 0,
        0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 32, 2, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 111, 0, 0, 0, 9, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 48, 2, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0,
        8, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 143, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 2, 0, 0, 0, 0, 0, 0, 6, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 148, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 5, 0, 0, 0, 0, 0, 0, 208, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
}
