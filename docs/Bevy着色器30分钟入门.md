# Bevy着色器30分钟入门

## 目录
1. [着色器基础概念](#1-着色器基础概念)
2. [Bevy中的着色器系统](#2-bevy中的着色器系统)
3. [什么时候使用着色器](#3-什么时候使用着色器)
4. [快速上手：第一个着色器](#4-快速上手第一个着色器)
5. [进阶示例](#5-进阶示例)
6. [调试技巧](#6-调试技巧)

## 1. 着色器基础概念

### 什么是着色器？
着色器（Shader）是运行在GPU上的小程序，用于控制图形渲染的各个阶段。它们决定了：
- 物体如何定位和变形（顶点着色器）
- 像素的颜色如何计算（片段着色器）

### 着色器管线
```
顶点数据 → [顶点着色器] → 图元装配 → 光栅化 → [片段着色器] → 像素输出
```

### WGSL简介
Bevy使用WGSL（WebGPU Shading Language）作为着色器语言：
- 类似Rust的语法
- 强类型系统
- 跨平台支持

## 2. Bevy中的着色器系统

### 核心组件

#### Material（材质）
```rust
// 自定义材质需要实现Material trait
#[derive(AsBindGroup, TypePath, Debug, Clone, Asset)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    texture: Option<Handle<Image>>,
}
```

#### Shader Asset（着色器资源）
```rust
// 加载着色器文件
let shader = asset_server.load("shaders/custom.wgsl");
```

#### RenderPipeline（渲染管线）
Bevy自动管理渲染管线，开发者只需关注Material和Shader。

## 3. 什么时候使用着色器

### 适合使用着色器的场景

#### 1. 特殊视觉效果
- **水面波纹**：需要实时计算波形
- **火焰效果**：需要噪声和颜色渐变
- **描边效果**：需要边缘检测
- **发光效果**：需要后处理

#### 2. 性能优化
- **GPU粒子系统**：大量粒子的位置计算
- **实例化渲染**：绘制大量相同物体
- **LOD系统**：根据距离调整细节

#### 3. 自定义渲染
- **非真实感渲染**：卡通渲染、素描效果
- **特殊材质**：全息效果、透明度处理
- **后处理效果**：模糊、色调映射

### 不需要着色器的场景
- 简单的2D精灵渲染
- 基础的3D模型显示
- 标准光照效果（Bevy内置）
- UI渲染

## 4. 快速上手：第一个着色器

### 步骤1：创建着色器文件
创建 `assets/shaders/color_wave.wgsl`：

```wgsl
// 顶点着色器输入
struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

// 顶点着色器输出
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world_position: vec3<f32>,
}

// 材质绑定
@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time: f32;

// 顶点着色器
@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // 添加波浪效果
    var pos = input.position;
    pos.y += sin(pos.x * 5.0 + time) * 0.1;
    
    output.clip_position = view.clip_from_world * vec4<f32>(pos, 1.0);
    output.uv = input.uv;
    output.world_position = pos;
    
    return output;
}

// 片段着色器
@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // 创建渐变色
    let gradient = sin(input.world_position.x + time) * 0.5 + 0.5;
    let color = mix(
        material_color,
        vec4<f32>(1.0, 0.0, 0.0, 1.0),
        gradient
    );
    
    return color;
}
```

### 步骤2：定义材质
```rust
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WaveMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    time: f32,
}

impl Material for WaveMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/color_wave.wgsl".into()
    }
    
    fn vertex_shader() -> ShaderRef {
        "shaders/color_wave.wgsl".into()
    }
}
```

### 步骤3：在Bevy中使用
```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<WaveMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_material_time)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaveMaterial>>,
) {
    // 创建相机
    commands.spawn(Camera3d::default());
    
    // 创建使用自定义着色器的网格
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(WaveMaterial {
            color: LinearRgba::rgb(0.0, 0.5, 1.0),
            time: 0.0,
        })),
    ));
}

fn update_material_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<WaveMaterial>>,
) {
    for (_, material) in materials.iter_mut() {
        material.time = time.elapsed_secs();
    }
}
```

## 5. 进阶示例

### 后处理效果
```rust
// 全屏后处理着色器
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PostProcessMaterial {
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,
    #[uniform(2)]
    intensity: f32,
}
```

### 粒子系统
```wgsl
// GPU粒子更新
@compute @workgroup_size(64)
fn update_particles(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    let index = id.x;
    var particle = particles[index];
    
    // 更新位置
    particle.position += particle.velocity * delta_time;
    
    // 应用重力
    particle.velocity.y -= 9.8 * delta_time;
    
    particles[index] = particle;
}
```

### 卡通渲染
```wgsl
// 卡通着色
@fragment
fn toon_fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 0.0));
    let n_dot_l = dot(input.normal, light_dir);
    
    // 量化光照为几个等级
    let light_intensity = floor(n_dot_l * 3.0) / 3.0;
    
    return vec4<f32>(
        material_color.rgb * max(light_intensity, 0.3),
        material_color.a
    );
}
```

## 6. 调试技巧

### 1. 可视化调试
```wgsl
// 输出UV坐标作为颜色
return vec4<f32>(input.uv, 0.0, 1.0);

// 输出法线作为颜色
return vec4<f32>(input.normal * 0.5 + 0.5, 1.0);
```

### 2. 热重载
```rust
// 开发时启用着色器热重载
app.add_plugins(DefaultPlugins.set(
    AssetPlugin {
        watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
        ..default()
    }
));
```

### 3. 性能分析
```rust
// 使用Bevy的诊断插件
app.add_plugins(FrameTimeDiagnosticsPlugin);
app.add_plugins(LogDiagnosticsPlugin::default());
```

## 实战建议

### 入门路径
1. **第1-10分钟**：理解着色器基本概念和GPU渲染管线
2. **第10-20分钟**：运行第一个简单着色器示例
3. **第20-30分钟**：修改着色器参数，观察效果变化

### 常见陷阱
- **坐标系统**：Bevy使用右手坐标系，Y轴向上
- **矩阵顺序**：变换矩阵是列主序
- **精度问题**：移动端可能不支持高精度浮点数

### 学习资源
- [Bevy着色器示例](https://github.com/bevyengine/bevy/tree/main/examples/shader)
- [WGSL规范](https://www.w3.org/TR/WGSL/)
- [The Book of Shaders](https://thebookofshaders.com/)（GLSL但概念通用）

## 总结

着色器是游戏开发中实现高级视觉效果的强大工具。在Bevy中：

1. **使用场景**：特效、优化、自定义渲染
2. **核心概念**：Material + Shader + RenderPipeline
3. **开发流程**：编写WGSL → 定义Material → 应用到实体

记住：不是所有效果都需要着色器，但掌握着色器能让你突破渲染的限制，创造独特的视觉体验！

## 下一步

- 尝试修改示例中的波浪参数
- 实现一个简单的描边效果
- 探索Bevy的内置着色器源码
- 学习计算着色器处理大规模数据