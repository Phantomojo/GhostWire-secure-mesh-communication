import bpy
import random
import math

# --- Configuration ---
NODE_COUNT = 15
AREA_SIZE = 15.0
CONNECTION_DISTANCE = 6.0
NODE_RADIUS = 0.4
PACKET_RADIUS = 0.15
ANIMATION_LENGTH = 250

# Colors (RGBA)
COLOR_NODE = (0.0, 0.8, 1.0, 1.0)  # Cyan/Neon Blue
COLOR_CONNECTION = (0.0, 0.3, 0.5, 1.0) # Faint Blue
COLOR_PACKET = (1.0, 0.4, 0.0, 1.0) # Orange
COLOR_BROKEN = (1.0, 0.0, 0.0, 1.0) # Red

def clear_scene():
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete()

def setup_camera():
    bpy.ops.object.camera_add(location=(0, -25, 20), rotation=(math.radians(55), 0, 0))
    camera = bpy.context.object
    camera.data.type = 'PERSP'
    bpy.context.scene.camera = camera

def setup_lighting():
    # Sun light
    bpy.ops.object.light_add(type='SUN', location=(0, 0, 10))
    sun = bpy.context.object
    sun.data.energy = 2.0
    
    # Area light for soft fill
    bpy.ops.object.light_add(type='AREA', location=(10, -10, 10))
    area = bpy.context.object
    area.data.energy = 500.0
    area.data.size = 10.0

def create_material(name, color, emission_strength=5.0):
    mat = bpy.data.materials.new(name=name)
    mat.use_nodes = True
    nodes = mat.node_tree.nodes
    links = mat.node_tree.links
    
    # Clear default nodes
    nodes.clear()
    
    # Create Emission node
    emission = nodes.new(type='ShaderNodeEmission')
    emission.inputs['Color'].default_value = color
    emission.inputs['Strength'].default_value = emission_strength
    
    # Create Output node
    output = nodes.new(type='ShaderNodeOutputMaterial')
    
    links.new(emission.outputs['Emission'], output.inputs['Surface'])
    return mat

def create_nodes(count, radius):
    nodes = []
    node_mat = create_material("NodeMat", COLOR_NODE, 8.0)
    
    for i in range(count):
        x = random.uniform(-AREA_SIZE/2, AREA_SIZE/2)
        y = random.uniform(-AREA_SIZE/2, AREA_SIZE/2)
        z = random.uniform(0, 2) # Slight height variation
        
        bpy.ops.mesh.primitive_uv_sphere_add(radius=radius, location=(x, y, z))
        node = bpy.context.object
        node.name = f"Node_{i}"
        node.data.materials.append(node_mat)
        
        # Animate appearance
        node.scale = (0, 0, 0)
        node.keyframe_insert(data_path="scale", frame=1)
        node.scale = (1, 1, 1)
        node.keyframe_insert(data_path="scale", frame=20 + i*2) # Staggered appearance
        
        nodes.append(node)
    return nodes

def create_connections(nodes, max_dist):
    connections = []
    conn_mat = create_material("ConnectionMat", COLOR_CONNECTION, 2.0)
    
    for i, node_a in enumerate(nodes):
        for j, node_b in enumerate(nodes):
            if i >= j: continue # Avoid duplicates
            
            dist = (node_a.location - node_b.location).length
            if dist < max_dist:
                # Create a cylinder between nodes
                # Math for cylinder transform is complex, using a curve is easier for visualization
                curve_data = bpy.data.curves.new('link_curve', type='CURVE')
                curve_data.dimensions = '3D'
                curve_data.bevel_depth = 0.05
                
                spline = curve_data.splines.new('POLY')
                spline.points.add(1)
                spline.points[0].co = (node_a.location.x, node_a.location.y, node_a.location.z, 1)
                spline.points[1].co = (node_b.location.x, node_b.location.y, node_b.location.z, 1)
                
                obj = bpy.data.objects.new(f"Link_{i}_{j}", curve_data)
                bpy.context.collection.objects.link(obj)
                obj.data.materials.append(conn_mat)
                
                # Store connection data
                connections.append({
                    'obj': obj,
                    'node_a': node_a,
                    'node_b': node_b,
                    'length': dist
                })
                
                # Animate appearance
                obj.hide_viewport = True
                obj.hide_render = True
                obj.keyframe_insert(data_path="hide_viewport", frame=1)
                obj.keyframe_insert(data_path="hide_render", frame=1)
                
                obj.hide_viewport = False
                obj.hide_render = False
                obj.keyframe_insert(data_path="hide_viewport", frame=40)
                obj.keyframe_insert(data_path="hide_render", frame=40)

    return connections

def animate_packet(start_node, end_node, start_frame, duration=60):
    packet_mat = create_material("PacketMat", COLOR_PACKET, 15.0)
    
    bpy.ops.mesh.primitive_uv_sphere_add(radius=PACKET_RADIUS, location=start_node.location)
    packet = bpy.context.object
    packet.name = "DataPacket"
    packet.data.materials.append(packet_mat)
    
    # Simple linear animation for now (teleport through mesh)
    # In a real pathfinding, we'd go node-by-node. 
    # Let's fake a hop for visual clarity.
    
    mid_point = (start_node.location + end_node.location) / 2
    mid_point.z += 2.0 # Arc path
    
    packet.location = start_node.location
    packet.keyframe_insert(data_path="location", frame=start_frame)
    
    packet.location = mid_point
    packet.keyframe_insert(data_path="location", frame=start_frame + duration // 2)
    
    packet.location = end_node.location
    packet.keyframe_insert(data_path="location", frame=start_frame + duration)
    
    # Scale animation (appear/disappear)
    packet.scale = (0, 0, 0)
    packet.keyframe_insert(data_path="scale", frame=start_frame)
    packet.scale = (1, 1, 1)
    packet.keyframe_insert(data_path="scale", frame=start_frame + 5)
    packet.scale = (0, 0, 0)
    packet.keyframe_insert(data_path="scale", frame=start_frame + duration)

def main():
    clear_scene()
    setup_camera()
    setup_lighting()
    
    # World Background
    world = bpy.context.scene.world
    world.use_nodes = True
    bg_node = world.node_tree.nodes['Background']
    bg_node.inputs[0].default_value = (0.01, 0.01, 0.02, 1) # Dark Blue/Black
    
    nodes = create_nodes(NODE_COUNT, NODE_RADIUS)
    connections = create_connections(nodes, CONNECTION_DISTANCE)
    
    # Animate Packets
    if len(nodes) > 1:
        for i in range(5): # Send 5 packets
            start = random.choice(nodes)
            end = random.choice(nodes)
            while start == end:
                end = random.choice(nodes)
            
            frame_start = 60 + i * 40
            animate_packet(start, end, frame_start)

    # Set frame range
    bpy.context.scene.frame_start = 1
    bpy.context.scene.frame_end = ANIMATION_LENGTH

if __name__ == "__main__":
    main()
