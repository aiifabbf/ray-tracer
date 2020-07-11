==========
ray-tracer
==========

.. default-role:: math

A toy ray-tracing renderer in Rust

.. figure:: cover.png
    :width: 100%

Quick start
===========

Download `this image <https://raytracing.github.io/images/earthmap.jpg>`_ and put it under root directory of this repo. It will be used as a texture map in the demo.

.. code-block:: bash

    cargo run --release > image.png

This will generate book 2's cover image.

``--release`` can significantly increase rendering speed.

If you are on windows, just ``cargo run --release`` and find a ``image.png`` under root directory.

To generate book 1's cover image (random spheres):

.. code-block:: bash

    cargo run --release --example book-one > image.ppm

To generate Cornell box:

.. code-block:: bash

    cargo run --release --example cornell-box > image.ppm

Features
========

-   lambertian, metal, dielectric (glass-like), light-emitting materials
-   sub-surface scattering inside constant density medium like fog and smoke
-   `bounding volume hierarchy <https://en.wikipedia.org/wiki/Bounding_volume_hierarchy>`_ to speedup ray-object intersection detection
-   sphere, rectangle, cube geometry
-   perspective camera with depth-of-field blurring effect

Build your own scene
====================

Build a blue smoke sphere with radius 70 and place it at `(360, 150, 145)`:

.. code-block:: rust

    let sphere = Arc::new(
        Sprite::builder()
            .geometry(ConstantMedium::new(Sphere::new(70.0).into(), 0.2).into())
            .material(Isotropic::new(Vec3::new(0.2, 0.4, 0.9)).into())
            .transform(Mat4::translation(Vec3::new(360.0, 150.0, 145.0)))
            .build(),
    );

Speed up ray-object intersection detection by constructing a bounding volume hierarchy:

.. code-block:: rust

    let sprites: Vec<Arc<dyn Bound<AxisAlignedBoundingBox>>> = unimplemented!();
    let bvh = BoundingVolumeHierarchyNode::new(sprites).unwrap();

Set up a perspective camera:

.. code-block:: rust

    let camera = PerspectiveCamera::new(
        eye, // eye position
        center, // what eye is looking at
        up, // camera's head up direction
        (40.0 as f64).to_radians(), // field of view in radians
        width as f64 / height as f64, // aspect ratio
        10.0, // focus distance
        0.01, // lens radius
    );

Get color at image position `(x, y)` (image coordinate origin is at lower-left corner, `+x` points rightward, `+y` points upward):

.. code-block:: rust

    let world: Arc<dyn Hit> = unimplemented!(); // your scene
    let u = (x as f64) / width as f64; // convert to camera coordinate
    let v = (y as f64) / height as f64;
    let ray = camera.ray(u, v);
    let pixel = color(&ray, world.as_ref(), 100); // ray scatters at most 100 times

References
==========

.. [book-i] `Ray Tracing in One Weekend <https://raytracing.github.io/books/RayTracingInOneWeekend.html>`_
.. [book-ii] `Ray Tracing: The Next Week <https://raytracing.github.io/books/RayTracingTheNextWeek.html>`_